# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpoQoFKm/cr3xGY_B9k5d_6Sw_1oC.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
