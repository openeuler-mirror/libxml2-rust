#!/bin/bash
echo "sys_test begin"
for file in ./*
do 
if [ -d "$file" ]
then
  cd $file
  /bin/bash $file.sh
  cd ..
fi
done
echo "sys_test end" 
