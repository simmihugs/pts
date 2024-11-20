from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import NoSuchElementException
import time
from datetime import date
import os
import sys
from dotenv import load_dotenv
import random
import argparse
import pandas as pd


class FluidDownloader:
    """
    Represent the download process of fluid data.
    """
    def __init__(self, prefix: str, username: str, password: str, file_name: str) -> None:
        self.prefix = prefix
        self.username = username
        self.password = password
        self.url = "https://secure.ses-ps.com/fluid/ui/dist/login.html"
        self.driver = self.create_webdriver()
        self.file_name = file_name


    def die(self):
        self.driver.close()
     
        
    def create_webdriver(self) -> webdriver.Chrome:
        """
        Creates the webdriver.
        """
        options = Options()
        options.add_argument('--headless')
        return webdriver.Chrome(options=options)
     
        
    def try_login(self) -> bool:
        """
        Try to login.
        
        Returns True on success. 
        """
        self.driver.get(self.url)
        self.driver.switch_to.frame('fluid-login-iframe')
        self.driver.find_element('name', 'user').send_keys(self.username)
        self.driver.find_element('id', 'pwd').send_keys(self.password)
        self.driver.find_element('id', 'loginBtn').click()

        login_success_full = False
        try:
            self.driver.refresh()
            time.sleep(3)
            self.driver.switch_to.default_content()
            return True
        except NoSuchElementException:
            return False

            
    def load_entries(self, number_of_entries=200) -> bool:
        """
        Loads 
        :param number_of_entries: in fluid.
        """
        try:
            advanced_search_button_settings = WebDriverWait(self.driver, 10).until( 
                EC.element_to_be_clickable((By.CSS_SELECTOR, ".icon-menu-context.search-action.settings")) 
            )
            advanced_search_button_settings.click()
        except Exception as exp:
            print(f"{exp=}")
            return False
        try: 
            slider = WebDriverWait(self.driver, 20).until( 
                EC.presence_of_element_located((By.CSS_SELECTOR, "input[type='range']")) 
            )
            self.driver.execute_script(f"arguments[0].value = {number_of_entries};", slider) 
            self.driver.execute_script("arguments[0].dispatchEvent(new Event('change'));", slider) 
        except Exception as exp:
            print(f"{exp=}")        
            return False
        
        try:
            self.driver.find_element('id', 'SearchButton').click()
            return True
        except Exception as exp:
            print(f"{exp=}")
            return False
      
        
    def export_entries(self) -> str:
        tmp = f'{self.file_name}_{str(date.today())}_{random.randint(1, 1000)}.xlsx'
        export_menu_button = WebDriverWait(self.driver, 10).until(
            EC.element_to_be_clickable((By.ID, 'ExportButton'))
        )
        export_menu_button.click()

        export_input = self.driver.find_element('id', 'customPrompt')
        export_input.clear()
        export_input.send_keys(tmp)

        export_button = WebDriverWait(self.driver, 10).until(
            EC.element_to_be_clickable((By.CSS_SELECTOR, 'button.big'))
        )
        export_button.click()

        export_path = f'{self.prefix}\\{tmp}'
        watining_time = 0
        while not os.path.exists(export_path) or watining_time > 10:   
            time.sleep(1)
            watining_time += 1      
        return tmp
            
            
    def download(self) -> str:
        if self.try_login():
            pass
        else:
            print("Could not login")
            sys.exit()
            
        if self.load_entries():
            pass
        else:
            print("Could not load entries")
            sys.exit()
            
        export_path = self.export_entries()
        return export_path       

def xlsx_to_csv(input_file, output_file):
    df = pd.read_excel(input_file)
    df.to_csv(output_file, index=False)

def main():    
    parser = argparse.ArgumentParser(description="Process some inputs.")
    parser.add_argument('-o', '--out', type=str, required=True, help='output file')  
    args = parser.parse_args()
    
    load_dotenv()

    username = os.getenv('LOGIN_USERNAME')
    password = os.getenv('LOGIN_PASSWORD')
    downloads = os.getenv('DOWNLOADS')   
    database = os.getenv('DATABASE') 
    
    fluid_downloader = FluidDownloader(prefix=downloads, username=username, password=password, file_name=args.out)
    export_file = fluid_downloader.download()
    fluid_downloader.die()
    
    in_file = f"{downloads}\\{export_file}"
    out_file = in_file.replace("xlsx", "csv")
    
    try:
        xlsx_to_csv(input_file=in_file, output_file=out_file)
    except Exception as exp:
        print(f"{exp=}")

    try:
        old_lines = []
        with open(f"{database}", "r+") as f:
            old_lines = f.readlines()

        new_lines = []
        with open(f"{out_file}", "+r") as f:
            new_lines = f.readlines()[1:]

        with open(f"{database}", "+w") as f:
            f.write(old_lines[0])
            f.writelines(new_lines)
            f.writelines(old_lines[1:])  
    except Exception as exp:
        print(f"{exp=}")

if __name__ == '__main__':
    main()
