// Zinc, the bare metal stack for rust.
// Copyright 2014 Lionel Flandrin <lionel@svkt.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! ISR data for tiva_c

use core::option::Option::{self, None};

const ISRCOUNT: usize = 139;

extern {
    fn isr_gpio_port_a();                      
    fn isr_gpio_port_b();                      
    fn isr_gpio_port_d();                      
    fn isr_gpio_port_e();                      
    fn isr_uart0();                      
    fn isr_uart1();                      
    fn isr_ssi0();                      
    fn isr_i2c0();                      
    fn isr_pwm_fault();                      
    fn isr_pwm_generator_0();                      
    fn isr_pwm_generator_1();                      
    fn isr_pwm_generator_2();                      
    fn isr_quadrature_encoder_0();                      
    fn isr_adc_sequence_0();                      
    fn isr_adc_sequence_1();                      
    fn isr_adc_sequence_2();                      
    fn isr_adc_sequence_3();                      
    fn isr_watchdog_timer();                      
    fn isr_timer_0_a();                      
    fn isr_timer_0_b();                      
    fn isr_timer_1_a();                      
    fn isr_timer_1_b();                      
    fn isr_timer_2_a();                      
    fn isr_timer_2_b();                      
    fn isr_analog_comparator_0();                      
    fn isr_analog_comparator_1();                      
    fn isr_analog_comparator_2();                      
    fn isr_system_control();                      
    fn isr_flash_control();                      
    fn isr_gpio_port_f();                      
    fn isr_gpio_port_g();                      
    fn isr_gpio_port_h();                      
    fn isr_uart2();                      
    fn isr_ssi1();                      
    fn isr_timer_3_a();                      
    fn isr_timer_3_b();                      
    fn isr_i2c1();                      
    fn isr_quadrature_encoder_1();                      
    fn isr_can0();                      
    fn isr_can1();                      
    fn isr_hibernate();                      
    fn isr_usb0();                      
    fn isr_pwm_generator_3();                      
    fn isr_udma_software_transfer();                      
    fn isr_udma_error();                      
    fn isr_adc1_sequence_0();                      
    fn isr_adc1_sequence_1();                      
    fn isr_adc1_sequence_2();                      
    fn isr_adc1_sequence_3();                      
    fn isr_gpio_port_j();                      
    fn isr_gpio_port_k();                      
    fn isr_gpio_port_l();                      
    fn isr_ssi2();                      
    fn isr_ssi3();                      
    fn isr_uart3();                      
    fn isr_uart4();                      
    fn isr_uart5();                      
    fn isr_uart6();                      
    fn isr_uart7();                      
    fn isr_i2c2();                      
    fn isr_i2c3();                      
    fn isr_timer_4_a();                      
    fn isr_timer_4_b();                      
    fn isr_timer_5_a();                      
    fn isr_timer_5_b();                      
    fn isr_wide_timer_0_a();                      
    fn isr_wide_timer_0_b();                      
    fn isr_wide_timer_1_a();                      
    fn isr_wide_timer_1_b();                      
    fn isr_wide_timer_2_a();                      
    fn isr_wide_timer_2_b();                      
    fn isr_wide_timer_3_a();                      
    fn isr_wide_timer_3_b();                      
    fn isr_wide_timer_4_a();                      
    fn isr_wide_timer_4_b();                      
    fn isr_wide_timer_5_a();                      
    fn isr_wide_timer_5_b();                      
    fn isr_fpu();                      
    fn isr_i2c4();                      
    fn isr_i2c5();                      
    fn isr_gpio_port_m();                      
    fn isr_gpio_port_n();                      
    fn isr_quadrature_encoder_2();                      
    fn isr_gpio_port_p0();                      
    fn isr_gpio_port_p1();                      
    fn isr_gpio_port_p2();                      
    fn isr_gpio_port_p3();                      
    fn isr_gpio_port_p4();                      
    fn isr_gpio_port_p5();                      
    fn isr_gpio_port_p6();                      
    fn isr_gpio_port_p7();                      
    fn isr_gpio_port_q0();                      
    fn isr_gpio_port_q1();                      
    fn isr_gpio_port_q2();                      
    fn isr_gpio_port_q3();                      
    fn isr_gpio_port_q4();                      
    fn isr_gpio_port_q5();                      
    fn isr_gpio_port_q6();                      
    fn isr_gpio_port_q7();                      
    fn isr_gpio_port_r();                      
    fn isr_gpio_port_s();                      
    fn isr_pwm_1_generator_0();                      
    fn isr_pwm_1_generator_1();                      
    fn isr_pwm_1_generator_2();                      
    fn isr_pwm_1_generator_3();                      
    fn isr_pwm_1_fault();                      
}

#[link_section=".isr_vector_nvic"]
#[no_mangle]
pub static NVIC_VECTOR: [Option<unsafe extern fn()>; ISRCOUNT] = [
    Some(isr_gpio_port_a),                      // GPIO Port A
    Some(isr_gpio_port_b),                      // GPIO Port B
    None,                                       // GPIO Port C
    Some(isr_gpio_port_d),                      // GPIO Port D
    Some(isr_gpio_port_e),                      // GPIO Port E
    Some(isr_uart0),                      // UART0 Rx and Tx
    Some(isr_uart1),                      // UART1 Rx and Tx
    Some(isr_ssi0),                      // SSI0 Rx and Tx
    Some(isr_i2c0),                      // I2C0 Master and Slave
    Some(isr_pwm_fault),                      // PWM Fault
    Some(isr_pwm_generator_0),                      // PWM Generator 0
    Some(isr_pwm_generator_1),                      // PWM Generator 1
    Some(isr_pwm_generator_2),                      // PWM Generator 2
    Some(isr_quadrature_encoder_0),                      // Quadrature Encoder 0
    Some(isr_adc_sequence_0),                      // ADC Sequence 0
    Some(isr_adc_sequence_1),                      // ADC Sequence 1
    Some(isr_adc_sequence_2),                      // ADC Sequence 2
    Some(isr_adc_sequence_3),                      // ADC Sequence 3
    Some(isr_watchdog_timer),                      // Watchdog timer
    Some(isr_timer_0_a),                      // Timer 0 subtimer A
    Some(isr_timer_0_b),                      // Timer 0 subtimer B
    Some(isr_timer_1_a),                      // Timer 1 subtimer A
    Some(isr_timer_1_b),                      // Timer 1 subtimer B
    Some(isr_timer_2_a),                      // Timer 2 subtimer A
    Some(isr_timer_2_b),                      // Timer 2 subtimer B
    Some(isr_analog_comparator_0),                      // Analog Comparator 0
    Some(isr_analog_comparator_1),                      // Analog Comparator 1
    Some(isr_analog_comparator_2),                      // Analog Comparator 2
    Some(isr_system_control),                      // System Control (PLL, OSC, BO)
    Some(isr_flash_control),                      // FLASH Control
    Some(isr_gpio_port_f),                      // GPIO Port F
    Some(isr_gpio_port_g),                      // GPIO Port G
    Some(isr_gpio_port_h),                      // GPIO Port H
    Some(isr_uart2),                      // UART2 Rx and Tx
    Some(isr_ssi1),                      // SSI1 Rx and Tx
    Some(isr_timer_3_a),                      // Timer 3 subtimer A
    Some(isr_timer_3_b),                      // Timer 3 subtimer B
    Some(isr_i2c1),                      // I2C1 Master and Slave
    Some(isr_quadrature_encoder_1),                      // Quadrature Encoder 1
    Some(isr_can0),                      // CAN0
    Some(isr_can1),                      // CAN1
    None,                      // Reserved
    None,                      // Reserved
    Some(isr_hibernate),                      // Hibernate
    Some(isr_usb0),                      // USB0
    Some(isr_pwm_generator_3),                      // PWM Generator 3
    Some(isr_udma_software_transfer),                      // uDMA Software Transfer
    Some(isr_udma_error),                      // uDMA Error
    Some(isr_adc1_sequence_0),                      // ADC1 Sequence 0
    Some(isr_adc1_sequence_1),                      // ADC1 Sequence 1
    Some(isr_adc1_sequence_2),                      // ADC1 Sequence 2
    Some(isr_adc1_sequence_3),                      // ADC1 Sequence 3
    None,                      // Reserved
    None,                      // Reserved
    Some(isr_gpio_port_j),                      // GPIO Port J
    Some(isr_gpio_port_k),                      // GPIO Port K
    Some(isr_gpio_port_l),                      // GPIO Port L
    Some(isr_ssi2),                      // SSI2 Rx and Tx
    Some(isr_ssi3),                      // SSI3 Rx and Tx
    Some(isr_uart3),                      // UART3 Rx and Tx
    Some(isr_uart4),                      // UART4 Rx and Tx
    Some(isr_uart5),                      // UART5 Rx and Tx
    Some(isr_uart6),                      // UART6 Rx and Tx
    Some(isr_uart7),                      // UART7 Rx and Tx
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    Some(isr_i2c2),                      // I2C2 Master and Slave
    Some(isr_i2c3),                      // I2C3 Master and Slave
    Some(isr_timer_4_a),                      // Timer 4 subtimer A
    Some(isr_timer_4_b),                      // Timer 4 subtimer B
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    None,                      // Reserved
    Some(isr_timer_5_a),                      // Timer 5 subtimer A
    Some(isr_timer_5_b),                      // Timer 5 subtimer B
    Some(isr_wide_timer_0_a),                      // Wide Timer 0 subtimer A
    Some(isr_wide_timer_0_b),                      // Wide Timer 0 subtimer B
    Some(isr_wide_timer_1_a),                      // Wide Timer 1 subtimer A
    Some(isr_wide_timer_1_b),                      // Wide Timer 1 subtimer B
    Some(isr_wide_timer_2_a),                      // Wide Timer 2 subtimer A
    Some(isr_wide_timer_2_b),                      // Wide Timer 2 subtimer B
    Some(isr_wide_timer_3_a),                      // Wide Timer 3 subtimer A
    Some(isr_wide_timer_3_b),                      // Wide Timer 3 subtimer B
    Some(isr_wide_timer_4_a),                      // Wide Timer 4 subtimer A
    Some(isr_wide_timer_4_b),                      // Wide Timer 4 subtimer B
    Some(isr_wide_timer_5_a),                      // Wide Timer 5 subtimer A
    Some(isr_wide_timer_5_b),                      // Wide Timer 5 subtimer B
    Some(isr_fpu),                      // FPU
    None,                      // Reserved
    None,                      // Reserved
    Some(isr_i2c4),                      // I2C4 Master and Slave
    Some(isr_i2c5),                      // I2C5 Master and Slave
    Some(isr_gpio_port_m),                      // GPIO Port M
    Some(isr_gpio_port_n),                      // GPIO Port N
    Some(isr_quadrature_encoder_2),                      // Quadrature Encoder 2
    None,                      // Reserved
    None,                      // Reserved
    Some(isr_gpio_port_p0),                      // GPIO Port P (Summary or P0)
    Some(isr_gpio_port_p1),                      // GPIO Port P1
    Some(isr_gpio_port_p2),                      // GPIO Port P2
    Some(isr_gpio_port_p3),                      // GPIO Port P3
    Some(isr_gpio_port_p4),                      // GPIO Port P4
    Some(isr_gpio_port_p5),                      // GPIO Port P5
    Some(isr_gpio_port_p6),                      // GPIO Port P6
    Some(isr_gpio_port_p7),                      // GPIO Port P7
    Some(isr_gpio_port_q0),                      // GPIO Port Q (Summary or Q0)
    Some(isr_gpio_port_q1),                      // GPIO Port Q1
    Some(isr_gpio_port_q2),                      // GPIO Port Q2
    Some(isr_gpio_port_q3),                      // GPIO Port Q3
    Some(isr_gpio_port_q4),                      // GPIO Port Q4
    Some(isr_gpio_port_q5),                      // GPIO Port Q5
    Some(isr_gpio_port_q6),                      // GPIO Port Q6
    Some(isr_gpio_port_q7),                      // GPIO Port Q7
    Some(isr_gpio_port_r),                      // GPIO Port R
    Some(isr_gpio_port_s),                      // GPIO Port S
    Some(isr_pwm_1_generator_0),                      // PWM 1 Generator 0
    Some(isr_pwm_1_generator_1),                      // PWM 1 Generator 1
    Some(isr_pwm_1_generator_2),                      // PWM 1 Generator 2
    Some(isr_pwm_1_generator_3),                      // PWM 1 Generator 3
    Some(isr_pwm_1_fault),                      // PWM 1 Fault
];
