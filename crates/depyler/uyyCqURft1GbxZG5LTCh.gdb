# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpEL303l/uyyCqURft1GbxZG5LTCh.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
