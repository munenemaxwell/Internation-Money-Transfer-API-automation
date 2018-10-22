<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Customer opt in to IMT service by providing location and accepting terms and condtions</description>
   <name>Opt_in</name>
   <tag></tag>
   <elementGuidId>beda8468-d8b4-400c-8648-405b882ab3ce</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ServiceRequest\&quot;: {\n    \&quot;Header\&quot;: {\n      \&quot;RequestRefID\&quot;: \&quot;drrrdrq\&quot;,\n      \&quot;Timestamp\&quot;: \&quot;201304021534k5\&quot;,\n      \&quot;OperationName\&quot;: \&quot;IMTOptIn\&quot;,\n      \&quot;OperationVersion\&quot;: \&quot;1\&quot;,\n      \&quot;SourceSystem\&quot;: \&quot;USSD\&quot;,\n      \&quot;Credentials\&quot;: {\n        \&quot;UserName\&quot;: \&quot;OdiDance\&quot;,\n        \&quot;Password\&quot;: \&quot;TDN0bTMxbjIwMTcwOTI0MDM0NTM1ODcz\&quot;\n      }\n    },\n    \&quot;Body\&quot;: {\n      \&quot;Data\&quot;: [\n                \n        {\&quot;Key\&quot;: \&quot;msisdn\&quot;, \&quot;Value\&quot;: \&quot;254705912645\&quot;},\n        {\&quot;Key\&quot;: \&quot;location\&quot;, \&quot;Value\&quot;: \&quot;Nairobi\&quot;}\n        \n       ]\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic T2RpRGFuY2U6VEROMGJUTXhiakl3TVRjd09USTBNRE0wTlRNMU9EY3o=</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.184.38.63:15533/customeroperations</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
