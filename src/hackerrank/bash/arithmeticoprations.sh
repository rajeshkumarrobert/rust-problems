#!/bin/bash

read X
read Y
Addition(){
	Z=$((X + Y));
	echo $Z;
}
Subtraction(){
        Z=$((X - Y));
        echo $Z;
}
Multiplication(){
        Z=$((X * Y));
        echo $Z;
}
Division(){
        Z=$((X / Y));
        echo $Z;
}
Addition
Subtraction
Multiplication
Division
