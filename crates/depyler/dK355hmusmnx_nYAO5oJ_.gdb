# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpu2lz9Y/dK355hmusmnx_nYAO5oJ_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
