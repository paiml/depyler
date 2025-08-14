# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpoEovW1/w1k0y.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
