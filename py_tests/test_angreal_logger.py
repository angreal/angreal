import angreal
import logging

def test_debug():
    logging.debug("test")
    logging.error("test")
    logging.info("test")
    logging.warning("test")
    logging.log(0,"test")