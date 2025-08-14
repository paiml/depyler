# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpVHqi3e/j_x_CH0u_6P3y.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
