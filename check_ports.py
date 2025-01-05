import socket

with open("env_vars.sh", "w") as file:
    tcp_count = 0
    udp_count = 0

    for port in range(5000, 6000):
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        result = sock.connect_ex(('127.0.0.1', port))
        if result != 0:  # Port is available
            if tcp_count < 28:
                file.write(f"export TCP_{tcp_count}={port}\n")
                tcp_count += 1
            elif udp_count < 28:
                file.write(f"export UDP_{udp_count}={port}\n")
                udp_count += 1
            if tcp_count >= 28 and udp_count >= 28:
                sock.close()
                break
        sock.close()