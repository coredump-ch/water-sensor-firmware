#include "mbed.h"

uint8_t sht3x_i2c_addr = 0x45<<1;

float calculate_temp(char msb, char lsb) {
    return -45.0f + 175.0f * (msb<<8 | lsb) / 65535.0f;
}

float calculate_humi(char msb, char lsb) {
    return 100.0f * (msb<<8 | lsb) / 65535.0f;
}

int send_command(I2C& i2c, uint8_t address, uint16_t command) {
    char cmd[2] = {char(command>>8), char(command & 0xFF)};
    return i2c.write(address, cmd, sizeof(cmd));
}

int main() {
    printf("Start the super awesome water temperature sensor reader\n");

    DigitalOut led1(LED1);
    DigitalOut led2(LED2);

    I2C i2c_0(p28, p27);
    I2C i2c_1(p9, p10);

    i2c_0.frequency(20000);
    i2c_1.frequency(20000);

    while(1) {
        led1 = 1;
        wait(0.2);

        // 0x2C06
        int error = send_command(i2c_0, sht3x_i2c_addr, 0x2C06);
        if (error) {
            printf("i2c_0.write failed: %i\n", error);
        }
        wait(0.5);

        char data[6] = {};
        error = i2c_0.read(sht3x_i2c_addr, data, 6);
        if (error) {
            printf("i2c_0.read failed: %i\n", error);
        }

        for(int i=0; i<6; ++i) {
            printf("%02x", data[i]);
        }
        float tmp = calculate_temp(data[0], data[1]);
        printf(" -> Temp = %.2f", tmp);

        float humi = calculate_humi(data[3], data[4]);
        printf(" Humi = %.2f\n", humi);

        led1 = 0;
        wait(0.2);
    }
}
