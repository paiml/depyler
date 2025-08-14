# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp65Huct/jn___j__8_Y.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
