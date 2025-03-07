import os
import sys
import shutil
import argparse
import logging
parent_path = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
sys.path.append(os.path.join(parent_path, 'Common'))
import log_helper
import constants
from args_validation_helper import check_arguments_type

@check_arguments_type
def print_usage(logger: logging.Logger):
    """ Prints the usage of script, which uses 'get_args()' method """
    logger.info("you must specify SolutionDir, TargetDir")
    logger.info("python %script%.py --solution_dir=XXX --target_dir=XXX")
    logger.info("current args: " + str(sys.argv))

@check_arguments_type
def get_args(logger: logging.Logger):
    """ Parses, validates and returns arguments, passed to the script """
    parser = argparse.ArgumentParser()
    parser.add_argument('--solution_dir', dest='solution_dir', action='store')
    parser.add_argument('--target_dir', dest='target_dir', action='store')
    args = parser.parse_args()
    if not args.solution_dir or not args.target_dir:
        logger.info('One or more arguments are not specified')
        print_usage(logger)
        logger.info(f'Args are: {args}')
        raise ValueError("One or more arguments are not specified")

    logger.info(f'Args are: {args}')
    return args

@check_arguments_type
def copy_files(solution_dir: str, target_dir: str):
    for arch, folder in constants.ARCH_TO_FOLDER_MAP.items():
        src_dir = constants.SRC_DIR_TEMPLATE.format(solution_dir, arch)
        dst_dir = os.path.join(target_dir, folder)
        os.makedirs(dst_dir, exist_ok=True)
        
        for file in constants.FILES:
            source_file = os.path.join(src_dir, file)
            if os.path.exists(source_file):
                shutil.copy2(source_file, dst_dir)
                logger.info(f"Copied {source_file} to {dst_dir}")
            else:
                logger.error(f"Warning: {source_file} not found")
                raise FileNotFoundError(f"File not found: {source_file}")

if __name__ == '__main__':
    try:
        logger, stream_handler, log_formatter = log_helper.setup_logger()
        logger.info("Starting Windows FLM adapter pre build event script")
        args = get_args(logger)
        logger.info("Copying files...")
        copy_files(args.solution_dir, args.target_dir)
        logger.info("Windows FLM adapter pre build event script finished")

    except Exception as ex:
        log_helper.final_log(stream_handler, logger, logging.ERROR, f"Windows FLM adapter pre build event script failed with an error {ex}")
        raise
    