# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp51bZEm/A6.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
