import os
import logging
import logging.config
import sys
import argparse
import constants
from args_validation_helper import check_arguments_type

@check_arguments_type
def set_file_logger_if_required(stream_handler: logging.StreamHandler, log_formatter: logging.Formatter, args: argparse.Namespace, logger: logging.Logger):
    """ Set log file and log formatter """
    if not args.log_to:
        return

    if os.path.exists(args.log_to):
        os.remove(args.log_to)

    logger.removeHandler(stream_handler)
    file_handler = logging.FileHandler(args.log_to, encoding='utf-8')
    file_handler.setFormatter(log_formatter)
    logger.addHandler(file_handler)
    logger.info(f'Current args are {args}')


@check_arguments_type
def final_log(stream_handler: logging.StreamHandler, logger: logging.Logger, level: int, msg: str):
    """ Print final log """
    if stream_handler not in logger.handlers:
        logger.addHandler(stream_handler)

    logger.log(level, msg)


def setup_logger():
    """ Set log settings"""
    logger = logging.getLogger(__name__)
    logger.setLevel(logging.DEBUG)
    stream_handler = logging.StreamHandler(sys.stdout)
    log_formatter = logging.Formatter(datefmt=constants.LOGGER_DATE_FORMAT, fmt=constants.LOGGER_FORMAT)
    logger.addHandler(stream_handler)
    return logger, stream_handler, log_formatter