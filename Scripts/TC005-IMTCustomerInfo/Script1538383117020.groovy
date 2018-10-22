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

//explicit import
import groovy.json.JsonSlurper

cust_info = WS.sendRequest(findTestObject('IMTCustomerInfo'))

WS.verifyResponseStatusCode(cust_info, 200)

def responseText = cust_info.getResponseText()

def cust_response_object = new groovy.json.JsonSlurper().parseText(responseText)

if(cust_response_object.ResponseHeader.ResponseCode=='4000'){
	
	WS.verifyMatch(cust_response_object.ResponseHeader.ResponseCode, '4000', false, FailureHandling.STOP_ON_FAILURE)

	WS.verifyMatch(cust_response_object.ResponseHeader.ResponseMsg, 'Success', false, FailureHandling.STOP_ON_FAILURE)

	WS.verifyMatch(cust_response_object.ResponseHeader.DetailedMsg, 'Customer is not registered', false, FailureHandling.STOP_ON_FAILURE)

	
}else{

	WS.verifyMatch(cust_response_object.ResponseHeader.ResponseCode, '4001', false, FailureHandling.STOP_ON_FAILURE)

	WS.verifyMatch(cust_response_object.ResponseHeader.ResponseMsg, 'Fail', false, FailureHandling.STOP_ON_FAILURE)

	WS.verifyMatch(cust_response_object.ResponseHeader.DetailedMsg, 'Customer already exist', false, FailureHandling.STOP_ON_FAILURE)


}




