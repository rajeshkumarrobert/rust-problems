#!/bin/bash

case ${1,,} in
	   Rooney | Rajesh | Admin )
		   echo "Hello My Friend Happy to see you!!"
		   ;;
	   New-friend )
		   echo "Hey Let's get to know about each other"
		   ;;
	   * )
		   echo "Introduce yourself stranger!!"
esac
