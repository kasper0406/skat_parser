import os
from faker import Faker

fake = Faker()

linenum = 0

def start_record():
    global linenum
    linenum += 1
    return str(linenum).zfill(7)

def record1000():
    f = []
    f.append(start_record())
    f.append('1000')

    f.append(fake.date(pattern='%Y%m%d'))
    f.append(fake.time(pattern='%H%M%S'))

    f.append(fake.numerify(text='########'))
    f.append(fake.numerify(text='########'))

    f.append(fake.random_element(elements=('01', '02')))

    f.append(''.ljust(5))
    f.append('0')

    f.append('Kapper Faker'.ljust(20))
    f.append('1.0'.ljust(20))

    f.append('BK Consult'.ljust(16))

    f.append('2.0')
    f.append('T')

    f.append(''.ljust(16))
    f.append(''.ljust(16))

    f.append('E')

    return ''.join(f)

def record2001():
    f = []

    f.append(start_record())
    f.append('2001')

    f.append(''.ljust(16))
    f.append(fake.numerify(text='########'))
    f.append(fake.random_element(elements=('A', ' ')))
    f.append('DKK')

    return ''.join(f)

def record2101():
    f = []

    f.append(start_record())
    f.append('2101')

    f.append(fake.numerify(text='##########'))
    f.append(''.ljust(8))
    f.append(''.ljust(15))

    f.append(fake.date(pattern='%Y%m%d'))
    f.append(fake.date(pattern='%Y%m%d'))

    f.append(''.ljust(24))

    f.append(fake.random_element(elements=('1', '2', '6', '7')))

    f.append(fake.date(pattern='%Y%m%d'))

    f.append(''.ljust(50))

    f.append(fake.random_element(elements=('R', ' ')))

    return ''.join(f)

def record2111():
    f = []

    f.append(start_record())
    f.append('2111')

    f.append(fake.numerify(text='####'))
    f.append(fake.numerify(text='############'))
    f.append(fake.date(pattern='%Y%m%d'))

    return ''.join(f)


def record9999():
    val = start_record()

    f = []
    f.append(val)
    f.append('9999')
    f.append(val)

    return ''.join(f)

with open("sample.txt", "w") as file:
    file.write(record1000() + os.linesep)
    file.write(record2001() + os.linesep)

    for i in range(fake.random_int(0, 10)):
        file.write(record2101() + os.linesep)
        for i in range(fake.random_int(0, 1)):
            file.write(record2111() + os.linesep)

    file.write(record9999() + os.linesep)
