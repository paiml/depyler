# GDB initialization script for Depyler debugging
# Source: /tmp/.tmptw6D3e/fJsM.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
