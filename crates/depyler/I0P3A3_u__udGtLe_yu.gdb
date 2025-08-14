# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpRwqbZY/I0P3A3_u__udGtLe_yu.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
