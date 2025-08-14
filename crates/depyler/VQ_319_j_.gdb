# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpVRbAhL/VQ_319_j_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
