import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

send_bank_response = WS.sendRequest(findTestObject('IMTSendToBank'))

WS.verifyResponseStatusCode(send_bank_response, 200)

def responseText = send_bank_response.getResponseText()

def send_bnk_response_object = new groovy.json.JsonSlurper().parseText(responseText)

WS.verifyMatch(send_bnk_response_object.ServiceResponse.ResponseHeader.ResponseCode, '4000', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(send_bnk_response_object.ServiceResponse.ResponseHeader.ResponseMsg, 'Success', false, FailureHandling.STOP_ON_FAILURE)

WS.verifyMatch(send_bnk_response_object.ServiceResponse.ResponseHeader.DetailedMsg, 'Success, Kindly wait to Enter your MPESA pin.', false, FailureHandling.STOP_ON_FAILURE)


