# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpt7ymsN/G__pqzH2.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
