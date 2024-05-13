#!/bin/bash

current_dir=$(pwd)
RUST_BACKTRACE=1

if [ -z "$1" ]; then

echo "Select the role to run:"
echo "  1) pool_sv2"
echo "  2) translator_sv2"
echo "  3) jd_client"
echo "  4) jd_server"

read -p "Enter choice [1-4]: " choice

case $choice in
    1) role="pool_sv2"
       config="config/pool.toml";;
    2) role="translator_sv2"
       config="config/tproxy.toml";;
    3) role="jd_client"
       config="config/jdc.toml";;
    4) role="jd_server"
       config="config/jds.toml";;    
    *) echo "Invalid choice. Exiting."
       exit 1
esac

echo "You have selected the role: $role"

# Number of times to run the command
max_runs=42

# Counter to keep track of how many times the command has been run
count=1


while [ $count -le $max_runs ]; do
    echo "Execution $count of $max_runs for $role"

    cd ../stratum-sidepool/roles
    # Run the cargo command for the specified role
    RUST_LOG=debug cargo run --bin $role -- -c $current_dir/$config
    
    # Check if cargo run was successful
    if [ $? -eq 0 ]; then
        echo "$role run completed successfully."
    else
        echo "$role run failed."
    fi

    # Increment the counter
    ((count++))
    
    # Wait for 30 seconds before the next run
    echo "Waiting for 30 seconds..."
    sleep 30
done

echo "Completed all $max_runs runs for $role."

else
   choice=$1
      
   case $choice in
      1) role="pool_sv2"
         config="config/pool.toml";;
      2) role="translator_sv2"
         config="config/tproxy.toml";;
      3) role="jd_client"
         config="config/jdc.toml";;
      4) role="jd_server"
         config="config/jds.toml";;    
      *) echo "Invalid choice. Exiting."
         exit 1
   esac

    cd ../roles
    # Run the cargo command for the specified role
    RUST_LOG=debug cargo run --bin $role -- -c $current_dir/$config
    
fi


