# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpuNhodA/z9eha.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
