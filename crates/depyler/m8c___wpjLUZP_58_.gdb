# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpIEMIC3/m8c___wpjLUZP_58_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
