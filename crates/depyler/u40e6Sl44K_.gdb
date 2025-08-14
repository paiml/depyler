# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpmQhJr4/u40e6Sl44K_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
