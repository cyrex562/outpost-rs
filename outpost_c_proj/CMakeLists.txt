cmake_minimum_required(VERSION 3.21)
project(outpost_c_proj C)

set(CMAKE_C_STANDARD 23)

add_executable(outpost_c_proj main.c file2.c file2.h program.h types.h structs.h globals.h sys_api.c sys_api.h utils.c utils.h structs_2.h entry.c entry.h ops_1.c ops_1.h ops_2.c ops_2.h ops_3.c ops_3.h string_defs.h ops_4.c ops_4.h ops_5.c ops_5.h win_ops.c win_ops.h mci_ops.c mci_ops.h ops_6.c ops_6.h structs_3.h ops_7.c ops_7.h)
