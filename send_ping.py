import socket

def send_redis_command(command):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    s.connect(('127.0.0.1', 6379))

    s.sendall(command.encode())

    response = s.recv(1024)
    print('Received', response.decode())

    s.close()

if __name__ == '__main__':
    send_redis_command("*2\r\n$4\r\nPING\r\n$6\r\nhello1\r\n")