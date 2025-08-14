# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpt7sISD/InNDMj7VN5u0_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
