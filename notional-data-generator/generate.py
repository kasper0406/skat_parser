import os
from faker import Faker

fake = Faker()

linenum = 0

def start_record():
    global linenum
    linenum += 1
    return str(linenum).zfill(7)

def reset():
    global linenum
    linenum = 0

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

def money_amount():
    return str(fake.random_number(digits=10)) + str(fake.random_number(digits=6))

def record3101():
    f = []

    f.append(start_record())
    f.append('3101')

    f.append(fake.random_element(elements=('F', 'B'))) # Forud / Bagud
    f.append(fake.numerify(text='################')) # Indberegning Id
    f.append(fake.numerify(text='################')) # Reference Id
    f.append(fake.date(pattern='%Y%m%d')) # Indberet start
    f.append(fake.date(pattern='%Y%m%d')) # Indberet slut
    f.append(fake.random_element(elements=('N', ' '))) # Nulangivelse
    f.append(fake.random_element(elements=('0000', '0059', '0400'))) # Felt-nr
    f.append(money_amount())
    f.append(fake.random_element(elements=('+', '-')))

    return ''.join(f)

def record4101():
    f = []

    f.append(start_record())
    f.append('4101')

    f.append(fake.numerify(text='################')) # Indberegning Id
    f.append(fake.numerify(text='################')) # Reference Id
    f.append(' ') # Filler
    f.append(fake.random_element(elements=('J', 'N'))) # Tilbagefoer SE
    f.append(fake.numerify(text='##########')) # CPR

    return ''.join(f)

def record5000():
    f = []

    f.append(start_record())
    f.append('5000')

    f.append(fake.random_element(elements=(' ', 'R'))) # Rettelse
    f.append(fake.numerify(text='################')) # Indberegning Id
    f.append(fake.numerify(text='################')) # Reference Id
    f.append(fake.date(pattern='%Y%m%d')) # Loen start dato
    f.append(fake.date(pattern='%Y%m%d')) # Loen slut dato
    f.append(fake.date(pattern='%Y%m%d')) # Dispositions dato
    f.append(' ') # Filler
    f.append(fake.random_element(elements=('F', 'B'))) # Forud / Bagud
    f.append(fake.random_element(elements=('   ', '020', '031', '032'))) # Groenlands kommune
    f.append(fake.random_element(elements=('00', '01', '03', '04', '05', '06', '07', '08', '09', '10', '11', '20', '24', '26'))) # Indkomsttype

    return ''.join(f)

def record6000():
    f = []

    f.append(start_record())
    f.append('6000')

    f.append('        ') # Filler
    f.append(fake.numerify(text='##########')) # CPR
    f.append(fake.numerify(text='########')) # SE / CVR
    f.append(fake.numerify(text='###############')) # Medarbejdsnr
    f.append(''.ljust(1 + 3 + 25)) # Filler
    f.append(fake.random_element(elements=('0002', '0004', '0005', '0006'))) # Indtaegtsart
    f.append(''.ljust(10)) # Produktions enhedsnr

    return ''.join(f)

def record6001():
    f = []

    f.append(start_record())
    f.append('6001')

    f.append(fake.random_element(elements=('0013', '0014', '0015', '0019'))) # Felt nr.
    f.append(money_amount())
    f.append(fake.random_element(elements=('+', '-')))

    return ''.join(f)

def record6002():
    f = []

    f.append(start_record())
    f.append('6002')

    f.append(fake.random_element(elements=('0024', '0043', '0099', '0143', '0209'))) # Felt nr.
    f.append(fake.numerify(text='##########')) # Kodefelt

    return ''.join(f)

def record6003():
    f = []

    f.append(start_record())
    f.append('6003')

    f.append(fake.random_element(elements=('0013', '0014', '0015', '0019'))) # Felt nr.
    f.append('X') # Afkrydningsfelt

    return ''.join(f)

def record6004():
    f = []

    f.append(start_record())
    f.append('6004')

    f.append(fake.random_element(elements=('0013', '0014', '0015', '0019'))) # Felt nr.
    f.append(fake.catch_phrase().ljust(58)) # Fritekst

    return ''.join(f)

def count(num_count, decimal_count):
    return str(fake.random_number(digits=num_count)) + str(fake.random_number(digits=decimal_count))

def record6005():
    f = []

    f.append(start_record())
    f.append('6005')

    f.append(fake.random_element(elements=('0083', '0200', '0203', '0211'))) # Felt nr.
    f.append(count(6, 2)) # Antal
    f.append(fake.random_element(elements=('+', '-')))

    return ''.join(f)

def record6102():
    f = []

    f.append(start_record())
    f.append('6102')

    f.append(money_amount()) # Nettoferiepenge
    f.append(fake.random_element(elements=('+', '-')))
    f.append(count(2, 2)) # Feriedage
    f.append(fake.random_element(elements=('+', '-')))
    f.append(fake.date(pattern='%Y')) # Ferieaar
    f.append(fake.date(pattern='%Y%m%d')) # Fratraeldelsesdato

    return ''.join(f)

def record6202():
    f = []

    f.append(start_record())
    f.append('6202')

    f.append(money_amount()) # Nettoferiepenge
    f.append(fake.random_element(elements=('+', '-')))
    f.append(count(2, 2)) # Feriedage
    f.append(fake.random_element(elements=('+', '-')))
    f.append(fake.date(pattern='%Y')) # Ferieaar
    f.append(fake.date(pattern='%Y%m%d')) # Fratraeldelsesdato

    return ''.join(f)

def record6111():
    f = []

    f.append(start_record())
    f.append('6111')
    
    f.append(fake.random_element(elements=('0100', '0200', '0350', '0400', '0600', '0610')))
    f.append(count(10, 0)) # Antal enheder
    f.append(fake.random_element(elements=('+', '-')))
    f.append(money_amount()) # Beloeb
    f.append(fake.random_element(elements=('+', '-')))

    return ''.join(f)

def record8001():
    f = []

    f.append(start_record())
    f.append('8001')

    f.append(fake.date(pattern='%Y%m%d')) # Birthday
    f.append(fake.random_element(elements=('1', '2', '3'))) # Gender
    f.append(fake.country_code()) # Country
    f.append(fake.name().ljust(40)) # Name
    f.append(fake.street_address().ljust(40)) # Address
    f.append(fake.postcode().ljust(9)) # Post code
    f.append(fake.city().ljust(35)) # City

    return ''.join(f)

def record9999():
    val = start_record()

    f = []
    f.append(val)
    f.append('9999')
    f.append(val)

    return ''.join(f)

def err_record0001():
    f = []
    f.append('0001')
    f.append('1337'.ljust(16))
    f.append(fake.date(pattern='%Y%m%d'))
    f.append(fake.time(pattern='%H%M%S'))
    f.append(fake.numerify(text='########'))
    f.append('Kapper Faker'.ljust(20))
    f.append('A')
    return ''.join(f)

def err_record0002(line, field, errcode, errtext, recid='0002'):
    f = []
    f.append(recid)
    f.append(str(line).zfill(7))
    f.append(str(field).zfill(2))
    f.append("ID#123456".ljust(16))
    f.append(fake.numerify(text='########'))
    f.append(str(errcode).zfill(6))
    f.append(errtext.ljust(300))
    f.append(fake.numerify(text="##########"))
    f.append(fake.numerify(text="########"))
    return ''.join(f)

def err_record9999():
    global linenum
    f = []
    f.append('9999')
    f.append(str(linenum + 2).zfill(7))
    return ''.join(f)

if __name__ == "__main__":
    with open("sample.txt", "w") as file:
        file.write(record1000() + os.linesep)
        file.write(record2001() + os.linesep)

        for i in range(fake.random_int(1, 10)):
            file.write(record2101() + os.linesep)
            file.write(record8001() + os.linesep)
            for i in range(fake.random_int(0, 1)):
                file.write(record2111() + os.linesep)

        file.write(record3101() + os.linesep)
        file.write(record4101() + os.linesep)
        
        for i in range(fake.random_int(10, 15)):
            file.write(record5000() + os.linesep)

            for j in range(fake.random_int(10, 500)):
                file.write(record6000() + os.linesep)
                for k in range(fake.random_int(0, 1)):
                    file.write(record8001() + os.linesep)
                for k in range(fake.random_int(1, 3)):
                    file.write(record6001() + os.linesep)
                for k in range(fake.random_int(0, 3)):
                    file.write(record6002() + os.linesep)
                for k in range(fake.random_int(0, 3)):
                    file.write(record6003() + os.linesep)
                for k in range(fake.random_int(0, 3)):
                    file.write(record6004() + os.linesep)
                for k in range(fake.random_int(0, 3)):
                    file.write(record6005() + os.linesep)
                for k in range(fake.random_int(0, 3)):
                    file.write(record6102() + os.linesep)
                for k in range(fake.random_int(0, 3)):
                    file.write(record6202() + os.linesep)
                for k in range(fake.random_int(0, 3)):
                    file.write(record6111() + os.linesep)

        file.write(record9999() + os.linesep)
    
    reset()

    with open("sample-errors.txt", "w") as file:
        file.write(err_record0001() + os.linesep)
        file.write(err_record0002(5, 3, 42, "Eksempelfejl 1") + os.linesep)
        file.write(err_record0002(8, 6, 1337, "Eksempelfejl 2") + os.linesep)
        file.write(err_record0002(10, 3, 13, "Eksempeladvarsel 1", "0003") + os.linesep)
        file.write(err_record9999() + os.linesep)
