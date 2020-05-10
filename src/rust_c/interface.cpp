#include <iostream>
typedef int (*onDataCallback_)(uint8_t *data, size_t len);

extern "C" int registerOnDataCallback(onDataCallback_ cb);

extern "C" void doSomething();

extern "C" int onDataCallback(uint8_t *data, size_t len) {
    std::cout << "called onData with size " << len << std::endl;
}

int main() {
    std::cout << "hello" << std::endl;
    registerOnDataCallback(&onDataCallback);
    doSomething();
    getchar();
    return 0;
}