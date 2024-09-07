DICT = {
    'un':'1','dos':'2','tres':'3','cuatro':'4','cinco':'5','seis':'6','siete':'7','ocho':'8','nueve':'9',
    'diez':'10','once':'11','doce':'12','trece':'13','catorce':'14','quince':'15','dieciseis':'16','diecisiete':'17','dieciocho':'18','diecinueve':'19',
    'veinte':'20','veintiuno':'21','veintidos':'22','veintitres':'23','veinticuatro':'24','veinticinco':'25','veintiseis':'26','veintisiete':'27','veintiocho':'28','veintinueve':'29',
    'treinta':'30',
    'cuarenta':'40',
    'cincuenta':'50',
    'sesenta':'60',
    'setenta':'70',
    'ochenta':'80',
    'noventa':'90',
    'cien':'100',
    'cientos':'100',
    'quinientos':'500',
    'setecientos':'700',
    'novecientos':'900',
    'mil':'1000',
    'y':'conjunction'
}

def str_to_num(strn):
    conjunction = False
    last_num = -1
    parts = strn.split(' ')
    num = 0
    for strn in parts:
        try:
            if strn in DICT:
                if DICT[strn] == 'conjunction':
                    conjunction = True
                elif DICT[strn] == '1000':
                    if last_num != -1:
                        num -= last_num
                        num += last_num * int(DICT[strn])
                        last_num = last_num * int(DICT[strn])
                    else:
                        num += int(DICT[strn])
                        last_num = int(DICT[strn])
                elif conjunction:
                    num += int(DICT[strn])
                    conjunction = False
                    last_num = -1
                else:
                    num += int(DICT[strn])
                    if last_num != -1:
                        last_num = last_num + int(DICT[strn])
                    else:
                        last_num = int(DICT[strn])
            else:
                idx = strn.find('cientos')
                if idx != -1:
                    num += int(DICT[strn[:idx]]) * int(DICT[strn[idx:]])
                    last_num = int(DICT[strn[:idx]]) * int(DICT[strn[idx:]])
                    continue
                raise Exception('Should not reach here')

        except Exception:
            raise Exception(f'Error: Conversion could not be performed. str `{strn}` not found in dictionary.')
        
    return str(num)

M_DICT = {
    'enero':'01',
    'febrero':'02',
    'marzo':'03',
    'abril':'04',
    'mayo':'05',
    'junio':'06',
    'julio':'07',
    'agosto':'08',
    'septiembre':'09',
    'octubre':'10',
    'noviembre':'11',
    'diciembre':'12'
}

# Example str: 'quince días del mes de enero del año dos mil veintiuno'
def get_date(strn):
    exc = Exception(f'Error: delimiter not found. Malformed str: `{strn}`')
    converted = ''

    # Day
    end_idx = strn.find('días')
    if end_idx == -1:
        raise exc
    converted += str_to_num(strn[:end_idx].strip()) + '/'

    # Month
    start_idx = strn.find('mes de')
    if start_idx == -1:
        raise exc
    start_idx += len('mes de')
    end_idx = strn.find('del año')
    if end_idx == -1:
        raise exc

    month = strn[start_idx:end_idx].strip()
    if month in M_DICT:
        converted += M_DICT[month] + '/'
    else:
        raise Exception(f'Error: month not found in month dictionary. Malformed str: `{strn}`')

    # Year
    start_idx = end_idx + len('del año')
    converted += str_to_num(strn[start_idx:].strip())

    return converted