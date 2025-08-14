# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpF5rLL9/BZ74F7Y90___85OH1.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
