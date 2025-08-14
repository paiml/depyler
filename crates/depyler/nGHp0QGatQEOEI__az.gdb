# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp9XOgjG/nGHp0QGatQEOEI__az.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
