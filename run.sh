#!/bin/bash
clear

echo -e "\e[33m\e[1mCompiling..."
echo -e "==============================================================================="
echo -e "\e[0m"

cargo run test.c

echo -e ""
echo -e "\e[1m\e[33m==============================================================================="
echo -e "Done Compiling!"
echo -e "\e[0m"

echo -e "\e[1m\e[32mRunning Program...\e[0m"

./out
RETURN_CODE=$?

echo -e "\e[1m\e[32mProgram exited with return code" $RETURN_CODE
echo -e "\e[0m"
