# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpDJNYw4/o19399SrGH88e2_E49XO.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
