from rebo import reboot


def stop():
    reboot(0xd000fce2)


if __name__ == '__main__':
    stop()
