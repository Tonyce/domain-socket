const net = require('net');
const { HelloRequest, HelloReply } = require('./proto/helloworld_pb');

const request = new HelloRequest();
request.setName("world");

var bytes = request.serializeBinary();

const data = encodeMsg(bytes)
console.log({ data })

// console.log({ bytes }, bytes.length);
// var messages = HelloRequest.deserializeBinary(bytes);
// console.log(messages.toObject())

// const lenBuf = int2BufEndianWay(bytes.length * 77)
// console.log({ lenBuf })

const socket = net.createConnection("/tmp/tokio.domain.socket");

socket.on('data', (data) => {
    console.log(data.toString())
})

// socket.write(bufStrToBuf('00 00 00 07 0a 05 77 6f 72 6c 64'));
setInterval(() => {
    socket.write(data);
}, 1000);


function bufStrToBuf(str) {
    let temp = str.split(' ').map(i => parseInt(i, 16));
    return Buffer.from(temp);
}

function int2BufEndianWay(x) {
    buf = Buffer.allocUnsafe(4)
    buf.writeUIntBE(x, 0, 4)
    return buf
}

function encodeMsg(msgByte) {
    const msgByteLen = msgByte.length;
    const lenBuf = int2BufEndianWay(msgByteLen);
    return Buffer.concat([lenBuf, msgByte])
}