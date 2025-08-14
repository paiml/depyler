# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpA4BQXZ/OdgrB6nU2nRE9w5H.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
