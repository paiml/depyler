# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpaBEk9c/X0UE6_1.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
