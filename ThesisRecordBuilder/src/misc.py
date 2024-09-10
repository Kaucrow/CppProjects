DICT = {
    '20':'veinte',
    '19':'diecinueve',
    '18':'dieciocho',
    '17':'diecisiete',
    '16':'dieciseis',
    '15':'quince',
    '14':'catorce',
    '13':'trece',
    '12':'doce',
    '11':'once',
    '10':'diez'
}

def num_to_str(num):
    if num in DICT:
        return DICT[num]
    else:
        raise Exception(f'Error: could not convert number to str. num {num} was not found in the dictionary.')