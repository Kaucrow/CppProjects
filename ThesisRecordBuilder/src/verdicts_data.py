from PIL import Image
import fitz
import pytesseract as tess
import numpy as np
import cv2
import re

def get_verdicts_data(verdict_path, data):
    pdf = fitz.open(verdict_path)
    first_page = pdf[0]
    pix = first_page.get_pixmap()
    im = np.frombuffer(pix.samples, dtype=np.uint8).reshape(pix.h, pix.w, pix.n)

    im = deskew(im)
    im = grayscale(im)

    im_cpy = im

    im = binarize(im, 190, 200)
    #im = remove_noise(im)

    color_img = cv2.cvtColor(im, cv2.COLOR_GRAY2BGR)
    x, y, w, h = cv2.boundingRect(get_main_contour(im))
    cv2.rectangle(color_img, (x, y), (x + w, y + h), (36, 255, 12), 2)
    #for c in cnts:
    #    x, y, w, h = cv2.boundingRect(c)
    #    cv2.rectangle(color_img, (x, y), (x + w, y + h), (36, 255, 12), 2)

    #display(color_img)

    tess_im = im_cpy[y: y+ h, x: x + w]
    color = [255, 255, 255]
    top, bottom, left, right = [50] * 4
    tess_im = cv2.copyMakeBorder(tess_im, top, bottom, left, right, cv2.BORDER_CONSTANT, value = color)
    im_h, im_w = tess_im.shape
    tess_im = cv2.resize(tess_im, (int(im_w * 1.2), int(im_h * 1.2)), interpolation = cv2.INTER_LINEAR)
    text = tess.image_to_string(tess_im)

    print(text)

    cipos = text.find('V-')
    if cipos == -1:
        raise Exception("Could not find the `V-` C.I. identifier.")

    cipos += 2
    citext = text[cipos:cipos+14]
    ci = re.sub('[^0-9]', '', citext)
    #ci = '00000001'

    student = None
    try:
        student = find_ci(data, ci)
    except Exception as err:
        #raise err
        return

    gradepos = text.find('con:')
    if gradepos == -1:
        raise Exception("Could not find the `con:` grade identifier.")

    gradepos += len('con:')
    gradetext = text[gradepos:gradepos+20]
    grade = re.sub('[^0-9]', '', gradetext)

    student['CALIFICACION'] = grade

    mentionpos = text.find('MENCI')
    if mentionpos != -1:
        print(verdict_path)
        mentiontext = text[mentionpos:mentionpos+20]
        words = mentiontext.split(' ')
        mention = words[1]
        student['MENCION'] = mention
    else:
        student['MENCION'] = None

def get_main_contour(im):
    def get_dilated(im, ksize):
        blur = cv2.GaussianBlur(im, (9, 9), 0)
        thresh = cv2.threshold(blur, 127, 255, cv2.THRESH_BINARY_INV + cv2.THRESH_OTSU)[1]
        kernel = cv2.getStructuringElement(cv2.MORPH_RECT, ksize)
        dilate = cv2.dilate(thresh, kernel, iterations = 1)
        return dilate
    
    def get_contours(im):
        cnts = cv2.findContours(im, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        cnts = cnts[0] if len(cnts) == 2 else cnts[1]
        cnts = sorted(cnts, key = cv2.contourArea, reverse=True)
        return cnts

    dilated = get_dilated(im, (23, 13))
    #display(dilated)
    cnts = get_contours(dilated)

    _, _, w, h = cv2.boundingRect(cnts[0])
    if not (h > 200):
        dilated = get_dilated(im, (23, 23))
        #display(dilated)
        cnts = get_contours(im)
        return cnts[0]
    elif not (w > 450):
        dilated = get_dilated(im, (33, 13))
        #display(dilated)
        cnts = get_contours(im)
        return cnts[0]
    else:
        return cnts[0]

def deskew(im):
    from deskew import determine_skew
    from typing import Tuple, Union

    def rotate(
            im: np.ndarray, angle: float, background: Union[int, Tuple[int, int]]
    ) -> np.ndarray:
        import math

        old_width, old_height = im.shape[:2]
        angle_radian = math.radians(angle)
        width = abs(np.sin(angle_radian) * old_height) + abs(np.cos(angle_radian) * old_width)
        height = abs(np.sin(angle_radian) * old_width) + abs(np.cos(angle_radian) * old_height)

        image_center = tuple(np.array(im.shape[1::-1]) / 2)
        rot_mat = cv2.getRotationMatrix2D(image_center, angle, 1.0)
        rot_mat[1, 2] += (width - old_width) / 2
        rot_mat[0, 2] += (height - old_height) / 2
        return cv2.warpAffine(im, rot_mat, (int(round(height)), int(round(width))), borderValue=background)

    angle = determine_skew(im)
    rotated = rotate(im, angle, (255, 255, 255))
    return rotated

def find_ci(data, ci):
    for thesis in data:
        if thesis['C.I.'] == ci:
            return thesis
    raise Exception('Could not find a thesis with C.I. ' + ci + '.')

def display(im):
    cv2.imshow('Page', im)
    cv2.waitKey(0)
    cv2.destroyAllWindows()

def thin_font(im):
    im = cv2.bitwise_not(im)
    kernel = np.ones((2, 2), np.uint8)
    im = cv2.erode(im, kernel, iterations = 1)
    im = cv2.bitwise_not(im)
    return(im)

def grayscale(im):
    return cv2.cvtColor(im, cv2.COLOR_BGR2GRAY)

def binarize(im, thresh, maxval, inv = False):
    if inv:
        thresh, im = cv2.threshold(im, thresh, maxval, cv2.THRESH_BINARY_INV)
        return im
    else:
        thresh, im = cv2.threshold(im, thresh, maxval, cv2.THRESH_BINARY)
        return im

def remove_noise(im):
    kernel = np.ones((1, 1), np.uint8)
    im = cv2.dilate(im, kernel, iterations = 1)
    kernel = np.ones((1, 1), np.uint8)
    im = cv2.erode(im, kernel, iterations = 1)
    im = cv2.morphologyEx(im, cv2.MORPH_CLOSE, kernel)
    im = cv2.medianBlur(im, 1)
    return im