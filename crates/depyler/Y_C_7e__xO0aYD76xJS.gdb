# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpLQdm7N/Y_C_7e__xO0aYD76xJS.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
