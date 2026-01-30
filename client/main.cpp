#include <iostream>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <cstring>

int main() {
    const char* env_path = getenv("XDG_RUNTIME_DIR");
    if (!env_path) {
        std::cerr << "XDG_RUNTIME_DIR not set" << std::endl;
        return -1;
    }

    std::string full_path_str = std::string(env_path) + "/aegis_bridge_guardian.sock";
    const char* socket_path = full_path_str.c_str();

    std::cout << "Attempting to connect to: " << socket_path << std::endl;

    int sock = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sock == -1) {
        perror("Socket initialization failed");
        return -1;
    }

    struct sockaddr_un addr = {};
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, socket_path, sizeof(addr.sun_path) - 1);

    if (connect(sock, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        perror("Connect failed");
        close(sock);
        return -1;
    }

    std::cout << "Connected successfully!" << std::endl;

    while(1){
        std::string message;
        std::cout << ">" << std::endl;
        std::getline(std::cin, message);

        if(message == "exit") break;

        send(sock, message.c_str(), strlen(message.c_str()), 0);

        char buffer[1024] = {0};
        size_t bytes_received = read(sock, buffer, sizeof(buffer) - 1);

        if(bytes_received < 0){
            perror("Failed to read response");
        }else if(bytes_received == 0){
            std::cout << "Socket closed" << std::endl;
        }else{
            buffer[bytes_received] = '\0';
            std::cout << "Response: " << buffer << std::endl;
        }

    }

    close(sock);
    return 0;
}
