const { Client, Server } = require("node-osc")
const { io } = require("socket.io-client")
const { exec } = require("child_process")
const iconv = require("iconv-lite")
const os = require("os")
const readline = require("readline")
const { existsSync } = require("fs")

const client = new Client("127.0.0.1", 9000)
const server = new Server(9001, "127.0.0.1")

const CHAT_TEXT = `No Text`
const HOSTNAME = os.hostname()

let enable_gpu = false

let statusData = {
    cpu: 0,
    ram: 0,
    ramData: ["0 Bytes", "0 Bytes"],
    disk: {
        idleTime: "0",
        read: "0 Bytes",
        write: "0 Bytes",
    },
}

if (existsSync("C:\\Windows\\System32\\nvidia-smi.exe")) {
    enable_gpu = true
    statusData.gpu = 0
    statusData.vmem = 0
    statusData.vmemData = ["0 Bytes", "0 Bytes"]
}

const sio = new io("wss://pcss.eov2.com")

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
})

let inputText = CHAT_TEXT

getPerf()

setInterval(() => getPerf(), 2000)

server.on("listening", () => {
    console.log("[LOG] OSC Server is listening.")
    rl.on("line", (line) => {
        if (line === "") return (inputText = CHAT_TEXT)
        inputText = line
    })
})

server.on("message", (msg) => {
    //console.log(`Message: ${msg}`);
})

sio.on("connect", () => {
    console.log("[LOG] Socket.io Client Connected")
})

sio.on("status", (msg) => {
    const status = Object.values(msg).find((pc) => pc.hostname === HOSTNAME)
    const cpu = getCPUPercent(status.cpu.cpus)
    const ram = getPercent(status.ram.free, status.ram.total)
    const ramData = [
        byteToData(status.ram.total - status.ram.free),
        byteToData(status.ram.total),
    ]
    const disk = statusData.disk

    const data = {
        cpu,
        ram,
        ramData,
        disk,
    }

    if (enable_gpu) {
        const gpu = status.gpu.usage
        const vmem = getPercent(status.gpu.memory.free, status.gpu.memory.total)
        const vmemData = [
            byteToData(
                (status.gpu.memory.total - status.gpu.memory.free) *
                    1024 *
                    1024,
            ),
            byteToData(status.gpu.memory.total * 1024 * 1024),
        ]

        data.gpu = gpu
        data.vmem = vmem
        data.vmemData = vmemData
    }

    statusData = data
})

setInterval(() => {
    const text = `${"["}${inputText}${"]"}`
    const gpu = enable_gpu
        ? ` | GPU: ${statusData.gpu}% | VMEM: ${statusData.vmem}%(${statusData.vmemData[0]}/${statusData.vmemData[1]})`
        : ""
    const res = `CPU: ${statusData.cpu}% | RAM: ${statusData.ram}%(${statusData.ramData[0]} / ${statusData.ramData[1]})${gpu} | DiskIO: ${statusData.disk.idleTime}%(r: ${statusData.disk.read} / w: ${statusData.disk.write}) | ${text}`
    send(res)
    console.log("[LOG]", res)
}, 3000)

function send(text) {
    client.send("/chatbox/input", text, true)
}

function getPercent(free, total) {
    return Math.floor((1 - free / total) * 100)
}

function getCPUPercent(result) {
    return result.length === 0
        ? 0
        : Math.floor(
              result.map((a) => a.cpu).reduce((a, b) => a + b) / result.length,
          )
}

function byteToData(a, b = 2) {
    if (0 >= a) return "0 Bytes"
    const c = 0 > b ? 0 : b,
        d = Math.floor(Math.log(a) / Math.log(1024))
    const datas = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"]
    return parseFloat((a / Math.pow(1024, d)).toFixed(c)) + " " + datas[d]
}

function iconvDecode(str = "") {
    const encoding = "shift-jis"
    const binaryEncoding = "binary"

    return iconv.decode(Buffer.from(str, binaryEncoding), encoding)
}

function getPerf() {
    exec(
        `TypePerf "\\PhysicalDisk(_Total)\\% Idle Time"  "\\PhysicalDisk(_Total)\\Disk Read Bytes/sec"  "\\PhysicalDisk(_Total)\\Disk Write Bytes/sec" -s ${HOSTNAME} -SC 1`,
        { encoding: "binary" },
        (error, stdout, stderr) => {
            if (stderr && error) return
            const res = iconvDecode(stdout)
            const parse = res
                .split("\r\n")
                .filter((v) => v)
                .map((v) => v.replace(/"/g, ""))[1]
                .split(",")

            const data = {
                idleTime: Math.floor(100 - parse[1]),
                read: byteToData(parse[2]),
                write: byteToData(parse[3]),
            }

            statusData.disk = data
        },
    )
}
