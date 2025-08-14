# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpspJWMn/X61_Q5s5_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
