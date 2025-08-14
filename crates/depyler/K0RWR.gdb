# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpaGgHXG/K0RWR.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
