const net = require('net');

const socket = net.createConnection("/tmp/tokio.domain.socket");

socket.on('data', (data) => {
    console.log(data.toString())
})

socket.write(bufStrToBuf('00 00 00 07 0a 05 77 6f 72 6c 64'));
setTimeout(() => {
    socket.write(bufStrToBuf('00 00 00 07 0a 05 77 6f 72 6c 64 00 00 00 08 0a 05 77 6f 72 6c 64 04'));
}, 1000);


function bufStrToBuf(str) {
    let temp = str.split(' ').map(i => parseInt(i, 16));
    return Buffer.from(temp);
}