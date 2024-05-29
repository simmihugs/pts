from pwinput import pwinput as prompt
from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import NoSuchElementException
import time
from datetime import date

def main():
    username = prompt(prompt='username: ')
    password = prompt(prompt='password: ')    

    options = Options()
    options.add_argument('--headless')

    driver = webdriver.Firefox(options=options)
    driver.get("https://secure.ses-ps.com/fluid/ui/dist/login.html")

    driver.switch_to.frame('fluid-login-iframe')
    driver.find_element('name', 'user').send_keys(username)
    driver.find_element('id', 'pwd').send_keys(password)
    driver.find_element('id', 'loginBtn').click()

    login_success_full = False
    try:
        driver.refresh()
        time.sleep(3)
        driver.switch_to.default_content()
        driver.find_element('id', 'SearchButton').click()
        login_success_full = True
        print("[+] Login successful")
    except NoSuchElementException:
        print("[!] Login failed")        

    if login_success_full:
        export_menu_button = WebDriverWait(driver, 10).until(
            EC.element_to_be_clickable((By.ID, 'ExportButton'))
        )
        export_menu_button.click()

        file_name = 'New_fluid_export_' + str(date.today())
        export_input = driver.find_element('id', 'customPrompt')
        export_input.clear()
        export_input.send_keys(file_name)

        export_button = WebDriverWait(driver, 10).until(
            EC.element_to_be_clickable((By.CSS_SELECTOR, 'button.big'))
        )
        export_button.click()

        print(f'Successfully exported: {file_name}.xlxs')
        driver.close()

if __name__ == '__main__':
    main()

