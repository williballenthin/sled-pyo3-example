import sys
import logging

import sp3e


logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)

db = sp3e.from_disk(sys.argv[1])

print(db)