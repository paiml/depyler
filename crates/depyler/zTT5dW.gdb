# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpw6M3TY/zTT5dW.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
