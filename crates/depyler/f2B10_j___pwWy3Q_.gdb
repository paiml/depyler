# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpjGkNoc/f2B10_j___pwWy3Q_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
